# aardwolf-interface
This is a dedicated repository for the User Interface (Front End) components of the Aardwolf Project :)

Ideally the interface being built here should be able to stand on its own.  This allows developers to play around, and test out ideas 
without having to build the full rust project.  Once the visual components have been vetted, they will of course need to be "backported" into the main repository
which is here: [Aardwolf-Social](https://gibhub.com/aardwolf-social/aardwolf).
<br />

One of the goals of the project overall is to keep it simple, and secure.  That means trying to minimize the use of iFrames, JavaScript, and other scripting 
languages.  Thankfully HTML5, and the primary element library [Bulma.io](https://bulma.io) are able to provide dynamic layouts using only HTML, and CSS. 
Additionally, icons provided by [ForkAwesome](https://forkawesome.github.io/Fork-Awesome/) (a FOSS alternative to that other popular icon library),
is also free of JavaScript. 
<br />


Banjo had another note planned for this, but he can't think of what the heck it was supposed to be.  Probably because he got distracted watching television.



**Directory Structure**<br />
There are two primary directories for the app `templates` which houses the bulk of the HTML files, and `web` which is where the styling, and static assets go.

```
├── templates                               -- The root for the HTML/Ructe templates
│   │
│   ├── asides                              -- Left-hand navigation menus 
│   │   ├── aside_settings.rs.html              -- Settings menu
│   │   └── aside_shortcuts.rs.html             -- Shortcuts menu
│   │
│   ├── containers                          -- Main container layouts
│   │   ├── container_preferences.rs.html       -- Preferences
│   │   └── container_profile.rs.html           -- Profile 
│   │
│   ├── elements                            -- Misc UI elements
│   │   ├── alert.rs.html                       -- Alerts
│   │   ├── icon.rs.html                        -- Icon
│   │   ├── input.rs.html                       -- Visible input
│   │   ├── password_input.rs.html              -- Hidden input
│   │   └── text_input.rs.html                  -- Text input
│   │
│   ├── email                               -- Templates used when sending e-mails
│   │   └── new_user_welcome.rs.html            -- Welcome email
│   │
│   ├── error                               -- Error pages 
│   │   └── http_error.html                     -- Basic Error page
│   │  
│   ├── home                                -- Homepage layouts
│   │   ├── home_feed.rs.html                   -- Home feed
│   │   ├── home_footer.rs.html                 -- Footer for logged in user
│   │   └── home_nav_top.rs.html                -- Top navigation for logged in user
│   │
│   ├── posts                               -- Templates related to posts
│   │   └── new_post.rs.html                    -- New post
│   │
│   ├── reply                               -- Reply layouts
│   │   └── reply_box.rs.html                   -- Base template for posting a reply 
│   │
│   ├── base_template.html                      -- Base template
│   ├── html_head.rs.html                       -- This is the <head> content
│   ├── sign_in.rs.html                         -- User sign-in page
│   └── sign_up.rs.html                         -- User sign-up page
```

```
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
│   ├── fonts          -- Font files for Forkawesome
│   └── forkawesome    -- Fork Awesome CSS (Not included)
│
└── themes          -- Theme files (future planning)
```  

**Developer Resources**

***InVision***<br />
Wireframes have been posted to InVision, which can also be used for advanced prototyping:<br />
[InVision - Aardwolf Wireframes](https://invis.io/H3OTASXPMSY)

***CodePen***<br />
Banjo also has a CodePen account here: <br />
[CodePen](https://codepen.io/BanjoFox/)
