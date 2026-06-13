# Description
* Refactor `CommonBtn`, `CommonList` to `CommonComponent` library. 
* The "excercise" page needs redesigned. It should be a separate component. The `FlashcardManagerView` should be triggered from inside this page.
* Use case is: 
  User clicks on "Exercise" button -> open `ExcerciseView` that has `ExerciseGrid` -> click on "Flashcard" button in `ExerciseGrid` -> Open `FlashcardManagerView`.
* Redesign 'ReviewPage'. It should have a list of Lessons at center. Clicking on a lesson label will open all tests for that lessons.
* User case is: 
  Use clicks on "Review Mode" -> open a list of lessons -> click on lesson label -> one a grid of tests -> click on a test -> open test view page for that test.
* Redesign `MatchingExerciseView`
# Implementation request
## Refactor and create a new common components:
- Refactor `CommonList`, `CommonBtn` into a `CommonComponent` library.
- Create a `CommonGrid` that holds 4x4 `CommonBtn`.
## Redesign "Exercise" view:
- Create `ExerciseView`that has 2 `CommonBtn` in row: "Generate Exercises" and "Export Lessons".
- Create `ExerciseGrid` that inherits `CommonGrid`. 
- The `CommonGrid` has 4x4 buttons. One is active and is "FlashCard". Clicking on FlashCard opens `FlashcardManagerView`
- Remove "FlashCard" button in `VocabularyPage` that sets `VocabularyAppLogic.active-view =2`.
## Redesign "ReviewPage":
- Component `LessonStackLabel` must be renamed to `LessonLabel`.
- Create `TestView` component that has `CommonGrid` that has all the tests.
- ReviewPage structure is good, but need some replacement:
  - Page heading should shows "Lesson name" instead of flashcard stack's name, `FlashcardAppLogic.flashcard-list[selected-review-index].stackname`.
  - Stack selection list should be changed to lesson selection list - ReviewPage should reuse `LessonStackList` but with all buttons removed, except the LessonStack. Suggest to use a flag to disable and remove buttons that allows edit/add/remove lessons.
```
HorizontalLayout {
            Rectangle {
                border-color: Tokens.border-color;
                border-radius: Tokens.border-radius;
                border-width: Tokens.border-width;
                VerticalLayout {
                    spacing: Tokens.spacing-sm;
                    padding: Tokens.padding-sm;

                    for lesson[i] in VocabularyAppLogic.lesson-list: HorizontalLayout {
                        spacing: Tokens.spacing-sm;

                        // Lesson label — checkable, mirrors FlashcardLabel usage.
                        LessonStackLabel {
                            horizontal-stretch: 1;
                            height: 36px;
                            lesson: lesson;
                            checkable: true;
                            checked: VocabularyAppLogic.selected-lesson-index == i;
                            clicked => {
                                VocabularyAppLogic.selected-lesson-index = i;         // it should open `TestView` in this case
                            }
                        }
                    // Delete button should be removed for ReviewPage
                    }
                }
            }
        }
```
- `LessonLabel` opens `TestView`. `TestView` has "Maching Test" button in its `CommonGrid`
- Clicking on "Maching Test" opens `MatchingExerciseView`.

## Redesign `MatchingExerciseView`
- Remove this text:
```
Text {
                text: matched-count + " / " + cards.length;
                font-size: Tokens.font-size-caption;
                color: Tokens.text-secondary;
                vertical-alignment: center;
            }
```
- It must have "Submit" button.
- The "Two-column tile grid" should have only 5 rows maximum, and doesn't overflow. Pagination button must show.
- User can match card but don't know if it is correct until user finish exercises by going to the last page and hit "Submit"
- User's Matched card should changes colors.
- Design a "Matching Exercise Result View" that is reuses `MatchingExerciseView` but doesn't have "Submit" button.
- Instead it has "Close" button and the text. Suggest to use boolean flag `result-view` to show appropriate components. 
```
Text {
                text: matched-count + " / " + cards.length;
                font-size: Tokens.font-size-caption;
                color: Tokens.text-secondary;
                vertical-alignment: center;
            }
```
- Correcly matched card should have light green border. Incorrecly matched card should have pick border.
- There is no need to store result in persistent data. No Rust logic is needed.
- Hitting "Close" button returns to `TestView`. 
- **Note** the logic that generates matching exercise from flashcard stack data is unchanged!
