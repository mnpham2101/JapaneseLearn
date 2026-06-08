# Bug description

All though there are default Vocabulary data, but there is no lesson list in the page `Vocabulary`. 
A console log has been added to init function; console log at startup indicate that the lesson-list size it zero:
```
Current lesson size:  0
```
The should load avaible lessons from `@vocabulary/ui/data/*.json` file. If there are multiple files, each should be a lessons, displayed in LessonStackList.

After resolving, a planuml file should be generated to document the call flow, and how lesson data are loaded.

# Analysis

# Solution