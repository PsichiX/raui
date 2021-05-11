# Overview

RAUI is a renderer agnostic UI system that is heavely inspired by **React**'s declarative UI composition and the **UE4 Slate** widget components system.

> 🗣 **Pronounciation:** RAUI is pronounced like **"ra"** ( the Egyptian god ) + **"oui"** ( french for "yes" ) — [Audio Example][pronounciation].

[pronounciation]: https://itinerarium.github.io/phoneme-synthesis/?w=/%27rawi/

The main idea behind RAUI architecture is to treat UI as another data source that you transform into your target renderable data format used by your rendering engine of choice.

## Architecture

### Application

It is the central point of user interrest. It performs whole UI processing logic. There you apply widget tree that wil be processed, send messages from host application to widgets and receive signals sent from widgets to host application.

```rust
let mut application = Application::new();
let tree = widget! {
    (app {
        title = (title_bar: {"Hello".to_owned()})
        content = (vertical_box [
            (#{"hi"} button: {"Say hi!".to_owned()})
            (#{"exit"} button: {"Close".to_owned()})
        ])
    })
};
let mut renderer = HtmlRenderer::default();
application.apply(tree);
let mapping = CoordsMapping::new(Rect {
    left: 0.0,
    right: 1024.0,
    top: 0.0,
    bottom: 576.0,
});
application.layout(&mapping, &mut DefaultLayoutEngine);
if let Ok(output) = application.render(&mapping, &mut renderer) {
    println!("OUTPUT: {}", output);
}
```

### Widget

Widgets are divided into three categories:

- **Widget Node** - used as source UI trees (variant that can be either a component, unit or none)
  ```rust
  widget! {
      (app {
          title = (title_bar: {"Hello".to_owned()})
          content = (vertical_box [
              (#{"hi"} button: {"Say hi!".to_owned()})
              (#{"exit"} button: {"Close".to_owned()})
          ])
      })
  }
  ```
- **Widget Component** - you can think of them as Virtual DOM nodes, they store:
  - pointer to _component function_ (that process their data)
  - unique _key_ (that is a part of widget ID and will be used to tell the system if it should carry its _state_ to next processing run)
  - boxed cloneable _properties_ data (if component is a function, then properties are function arguments)
  - _listed slots_ (simply: widget children)
  - _named slots_ (similar to listed slots: widget children, but these ones have names assigned to them, so you can access them by name instead of by index)
- **Widget Unit** - an atomic element that renderers use to convert into target renderable data format for rendering engine of choice.
  ```rust
  widget!{{{
      TextBoxNode {
          text: "Hello World".to_owned(),
          ..Default::default()
      }
  }}}
  ```

### Component Function

Component functions are static functions that transforms input data (properties, state or neither of them) into output widget tree (usually used to simply wrap another components tree under one simple component, where at some point the simplest components returns final _widget units_).
They work together as a chain of transforms - root component applies some properties into children components using data from its own properties or state.

```rust
#[derive(Debug, Default, Copy, Clone)]
struct AppProps {
    pub index: usize,
}
implement_props_data!(AppProps);

widget_component! {
    app(props, named_slots) {
        unpack_named_slots!(named_slots => { title, content });
        let index = props.read::<AppProps>().map(|p| p.index).unwrap_or(0);

        widget! {
            (#{index} vertical_box [
                {title}
                {content}
            ])
        }
    }
}

widget_component! {
    vertical_box(key, listed_slots) {
        let items = listed_slots
            .into_iter()
            .map(|slot| FlexBoxItemNode {
                slot: slot.try_into().expect("Cannot convert slot to WidgetUnit!"),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        widget! {{{
            FlexBoxNode {
                items,
                ..Default::default()
            }
        }}}
    }
}
```

This may bring up a question: _**"If i use only functions and no objects to tell how to visualize UI, how do i keep some data between each render run?"**_.
For that you use _states_. State is a data that is stored between each processing calls as long as given widget is alive (that means: as long as widget id stays the same between two processing calls, to make sure your widget stays the same, you use keys - if no key is assigned, system will generate one for your widget but that will make it possible to die at any time if for example number of widget children changes in your common parent, your widget will change its id when key wasn't assigned).
Some additional notes: While you use _properties_ to send information down the tree and _states_ to store widget data between processing cals, you can communicate with another widgets and host application using messages and signals!
More than that, you can use hooks to listen for widget life cycle and perform actions there.
It's worth noting that state uses _properties_ to hold its data, so by that you can for example attach multiple hooks that each of them uses different data type as widget state, this opens the doors to be very creative when combining different hooks that operate on the same widget.

```rust
#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
struct ButtonState {
    pub pressed: bool,
}
implement_props_data!(ButtonState, "ButtonState");

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ButtonAction {
    Pressed,
    Released,
}

widget_hook! {
    use_button(life_cycle) {
        life_cycle.mount(move |context| {
            drop(context.state.write(ButtonState { pressed: false }));
        });

        life_cycle.change(move |context| {
            for msg in context.messenger.messages {
                if let Some(msg) = msg.downcast_ref::<ButtonAction>() {
                    let pressed = match msg {
                        ButtonAction::Pressed => true,
                        ButtonAction::Released => false,
                    };
                    drop(context.state.write(ButtonState { pressed }));
                    drop(context.signals.write(*msg));
                }
            }
        });
    }
}

widget_component! {
    button(key, props) [use_button] {
        let label = props.read_cloned_or_default::<String>();

        widget!{
            (#{key} text: {label})
        }
    }
}
```

### Hooks

Hooks are used to put common widget logic into separate functions that can be chained in widgets and another hooks (you can build a reusable dependency chain of logic with that).
Usually it is used to listen for life cycle events such as mount, change and unmount, additionally you can chain hooks to be processed sequentially in order they are chained in widgets and other hooks.

```rust
widget_hook! {
    use_empty {}
}

widget_hook! {
    use_button(life_cycle) [use_empty] {
        life_cycle.mount(move |context| {
            drop(context.state.write(ButtonState { pressed: false }));
        });

        life_cycle.change(move |context| {
            for msg in context.messenger.messages {
                if let Some(msg) = msg.downcast_ref::<ButtonAction>() {
                    let pressed = match msg {
                        ButtonAction::Pressed => true,
                        ButtonAction::Released => false,
                    };
                    drop(context.state.write(ButtonState { pressed }));
                    drop(context.signals.write(*msg));
                }
            }
        });
    }
}

widget_component! {
    button(key, props) [use_button] {
        widget!{
            (#{key} text: {props})
        }
    }
}
```

What happens under the hood:

- Application calls `button` on a node
  - `button` calls `use_button` hook
    - `use_button` calls `use_empty` hook
  - `use_button` logic is executed
- `button` logic is executed

### Layouting

_**TODO**_

RAUI exposes API (`Application::layout()`) to allow use of virtual-to-real coords mapping and custom layout engines to perform widget tree positioning data, which is later used by custom UI renderers to specify boxes where given widgets should be placed.
Every call to perform layouting will store a layout data inside Application, you can always access that data at any time.
There is a `DefaultLayoutEngine` that does this in a generic way.
If you find some part of its pipeline working different than what you've expected, feel free to create your custom layout engine!

```rust
let mut application = Application::new();
application.apply(tree);
application.process();
let mapping = CoordsMapping::new(Rect {
    left: 0.0,
    right: 1024.0,
    top: 0.0,
    bottom: 576.0,
});
if application.layout(&mapping, &mut DefaultLayoutEngine).is_ok() {
    println!("LAYOUT:\n{:#?}", application.layout_data());
}
```

### Interactivity

_**TODO**_

RAUI allows you to ease and automate interactions with UI by use of Interactions Engine - this is just a struct that implements `perform_interactions` method with reference to Application, and all you should do there is to send user input related messages to widgets.
There is `DefaultInteractionsEngine` that covers widget navigation, button and input field - actions sent from input devices such as mouse (or any single pointer), keyboard and gamepad. When it comes to UI navigation you can send raw `NavSignal` messages to the default interactions engine and despite being able to select/unselect widgets at will, you have typical navigation actions available: up, down, left, right, previous tab/screen, next tab/screen, also being able to focus text inputs and send text input changes to focused input widget. All interactive widget components that are provided by RAUI handle all `NavSignal` actions in their hooks, so all user has to do is to just activate navigation features for them (using `NavItemActive` unit props).
RAUI integrations that want to just use use default interactions engine should make use of this struct composed in them and call its `interact` method with information about what input change was made.
There is an example of that feature covered in Tetra integration crate (`TetraInteractionsEngine` struct).

**NOTE: Interactions engines should use layout for pointer events so make sure that you rebuild layout before you perform interactions!**

```rust
let mut application = Application::new();
let mut interactions = DefaultInteractionsEngine::new();
interactions.interact(Interaction::PointerMove(200.0, 100.0));
interactions.interact(Interaction::PointerDown(PointerButton::Trigger, 200.0, 100.0));
application.apply(tree);
application.process();
let mapping = CoordsMapping::new(Rect {
    left: 0.0,
    right: 1024.0,
    top: 0.0,
    bottom: 576.0,
});
application.layout(&mapping, &mut DefaultLayoutEngine);
application.interact(&mapping, &mut interactions);
```

## Media

- [`RAUI + Tetra In-Game`](https://github.com/PsichiX/raui/tree/master/demos/in-game)
  An example of an In-Game integration of RAUI with custom Material theme, using Tetra as a renderer.

  ![RAUI + Tetra In-Game](https://github.com/PsichiX/raui/blob/master/media/raui-tetra-in-game-material-ui.gif?raw=true)

- [`RAUI + Tetra todo app`](https://github.com/PsichiX/raui/tree/master/demos/todo-app)
  An example of TODO app with Tetra renderer and dark theme Material component library.

  ![RAUI + Tetra todo app](https://github.com/PsichiX/raui/blob/master/media/raui-tetra-todo-app-material-ui.gif?raw=true)
