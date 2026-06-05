# General Architecture
- Follow MVC architecture, but keep the common folder structure as other Slint-Rust applications: 
  - Slint components must be in together, but keep the folder name as 'ui/components/'. The entry Slint component must be in folder 'ui'
  - All public model data struct should be defined in 'ui/model/'
  - All backend business logic should be in 'src/'
- If the Slint components and its Rust backend logic are built as library, keep them in '/lib'. 
- Folder structure: 
```
src/
  main.rs                       # entry point only — no business logic here
ui/
  main_window.slint             # root window, page routing
  components/                   # main application components - one component per file
  ui/styles                     # reusable style definitions
  model/                        # main application data models
  pages/                        # page-level components (study_page, review_page)
lib/
  libA/                           # Slint component and backend logic built as library
    src/lib.rs                    # entry point to init function
    src/*.rs                      # Rust backend logic supporting UI manipulation
    ui/components                 # reusable widgets — one component per file 
    ui/model                      # reusable data models written in Slint
    ui/main_lib.slint             # entry slint library file - export the library components and models                
    build.rs                      # entry point to build library
    Cargo.toml                    # define building codes as package
  libB/                           # Rust logic built as service library
    src/lib.rs                    # Rust backend logic
    src/*.rs                      # Rust backend logic, could include helper functions, classes, etc 
    main.rs                       # entry point to build library
    Cargo.toml                    # define building codes as package
build.rs
Cargo.toml
```