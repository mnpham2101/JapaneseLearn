---
agent: speckit.clarify
---
# Design common button:
* check slint document for behavior of `enabled`, `checked`, `checkable`, `pressed` properties and add same properties and logic to common_button. Keep the name, and comments:
```
// button could be disable. Callback can only be invoked when enabled is true.
in-out property <bool> enabled <=> ta.enabled: true;
// if the button is clicked, button state changes. Useful for toggle button, in flashcard
in-out property <bool> checked: falsed;
// if the button is checkable, it can be checked or unchecked when clicked. Otherwise, it behaves like a normal button without checked state. Usefull for mutual exclusive button group, in flashcard stack list.
in property <bool> checkable: false;
// if the button is pressed, it is being clicked. Useful for visual feedback when user clicks and holds the button.
out property <bool> pressed: false;
ta:=TouchArea {
  clicked => {
    if (checkable) {
        checked = !checked;
    }
  }
}
```
* implement "accessible" properties  in slint.
* all specialized buttons (e.g., flashcard known/unknown toggle, stack selection) should inherit from the common button and extend it with specific properties and behaviors as needed.