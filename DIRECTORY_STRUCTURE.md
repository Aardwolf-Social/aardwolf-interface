## HTML/CSS TEMPLATE FILES
```
templates                               -- The root for the HTML/Ructe templates
│
├── asides                              -- Left-hand navigation menus 
│   ├── aside_settings.html              -- Settings menu
│   └── aside_shortcuts.html             -- Shortcuts menu
│
├── containers                          -- Main container layouts
│   ├── container_preferences.html       -- Preferences
│   └── container_profile.html           -- Profile 
│
├── elements                            -- Misc UI elements
│   ├── alert.html                       -- Alerts
│   ├── icon.html                        -- Icon
│   ├── input.html                       -- Visible input
│   ├── password_input.html              -- Hidden input
│   └── text_input.html                  -- Text input
│
├── email                               -- Templates used when sending e-mails
│   └── new_user_welcome.html            -- Welcome email
│
├── error                               -- Error pages 
│   └── http_error.html                     -- Basic Error page
│  
├── home                                -- Homepage layouts
│   ├── home_feed.html                   -- Home feed
│   ├── home_footer.html                 -- Footer for logged in user
│   └── home_nav_top.html                -- Top navigation for logged in user
│
├── posts                               -- Templates related to posts
│   └── new_post.html                    -- New post
│
├── reply                               -- Reply layouts
│   └── reply_box.html                   -- Base template for posting a reply 
│
├── base_template.html                   -- Base template
├── html_head.html                       -- This is the <head> content
├── sign_in.html                         -- User sign-in page
└── sign_up.html                         -- User sign-up page

```
## STYLISTIC AND STATIC ASSETS
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
│   ├── bulma          -- Bulma framework
│   └── forkawesome    -- Fork Awesome icons
│
└── themes          -- Theme files (future planning)
```  
