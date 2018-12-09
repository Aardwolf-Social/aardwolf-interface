# aardwolf-interface
This is a dedicated repository for the User Interface (Front End) components of the Aardwolf Project :)

Ideally the interface being built here should be able to stand on its own.  This allows developers to play around, and test out ideas 
without having to build the full rust project.  Once the visual components have been vetted, they will of course need to be "backported" into the main repository
which is here: [Aardwolf-Social](https://gibhub.com/aardwolf-social/aardwolf).
<br />

One of the goals of the project overall is to keep it simple, and secure.  That means trying to minimize the use of iFrames, JavaScript, and other scripting 
languages.  Thankfully HTML5, and the primary element library (Bulma.io)[https://bulma.io] are able to provide dynamic layouts using only HTML, and CSS. 
Additionally, icons provided by (ForkAwesome)[https://forkawesome.github.io/Fork-Awesome/] (a FOSS alternative to that other popular icon library),
is also free of JavaScript. 
<br />

Banjo had another note planned for this, but he can't think of what the heck it was supposed to be.  Probably because he got distracted watching television.



** Directory Structure **
templates/      -- Template root (template base, un-authenticated templates go here)
├── asides      -- Left-hand menus
├── email       -- Email templates
├── home        -- Templates for logged-in users (main content, top nav, footer, etc.)
├── posts       -- Templates related to user posts (new post, reply, etc.)
└── widgets     -- Reusable elements (text/password inputs, buttons, etc.)

web
├── _sass           -- SASS files (SCSS syntax)
│   ├── base            -- Global SCSS 
│   ├── external        -- reset.scss (and similar)
│   ├── includes        -- footer, header
│   └── layouts         -- structural styles (Bulma, and overrides)
│
├── emoji           -- Emoji images (future planning)
├── images          -- Static images
├── javascript      -- JavaScript files (if needed)
├── static          -- Other static files
│
├── css             -- Compiled CSS
│   ├── fonts               -- Font files for Forkawesome
│   └── forkawesome-1.1.5   -- Fork Awesome CSS
│
└── themes          -- Theme files (future planning)
  

** Developer Resources **

Wireframes have been posted to InVision, which can also be used for advanced prototyping:
[InVision - Aardwolf Wireframes](https://invis.io/H3OTASXPMSY)

