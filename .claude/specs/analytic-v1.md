# Goals:
- The application **should** track study sessions: date, stack studied, total cards, known count, following the specs in `@analytic-v1.md`
- The application **should** display progress analytics per stack: known vs unknown bar, cards studied over time. `@analytic-v1.md`

# Implementation Request
- `FlashcardAppLogic.known-count` and  `FlashcardAppLogic.total-count` displays incorrect values. 
- `CommonBtn` is not working correctly. Clicking on the button doesn't change `FlashcardAppLogic.known-count`.
-  Clicking on the button should only mark the current card as `known` or unmark it.
- Create a separate Rust module `StudyAnalytic` and provides business logic, make it a library:
  - to calculate known words vs total word and future request. Implement it in a separate class `VocabularyStudyAnalytic`.
  - to calculate days study and calendar to test day. User can provide target testing day.
  - future request extends either the module `StudyAnalytic` by adding new class or extend the class `VocabularyStudyAnalytic`.
- Create a new page `Analytic`, provided by Slint lib `StudyAnalytic`. The Slint lib should only provide Slint components, not the Rust logic defined in `StudyAnalytic`.
  - Slint lib `StudyAnalytic` provides "Views":
    - calendar that marks the day we study, and prints "days until tests".
    - lines charts that includes the `known` words vs `total` words