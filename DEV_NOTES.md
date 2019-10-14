## Developer Notes

**A note from Banjo**
Unfortunately this repo hasn't gotten nearly as much attention as the main repo.  Hoping to change that :)
This document is for the purpose of trying to explain the design as originally planned.  Do not hesitate to suggest changes though!
- Banjo

**Directory Structure**
* The `/templates` directory is for the HTML
* The `/web` directory is used to store all of the static assets
* The `/demo_files` directory should be used for any files which are for demonstration/testing purposes only.
* Additional directory etails can be tracked in the DIRECTORY_STRUCTURE.md

**Web Layout**
There should be two SASS folder structures.
* `/web/_scss` will be used for the application layouts
* `/web/themes` will be used for user-friendly theme files so that Admins/Users can the color schemes

**File Layouts** 
Any of the `/demo` files are likely to be combinations of templates, and are hopefully documented verbosely.  The `/demo/demo.html` has examples 
of how Banjo has been doing HTML comments.  Keeping the comment styles identical would be ideal, but there is also room for change.

Files in the other directories should be limited to only code that is required for that unique part of the application.  Short, in-line comments
are still perfectly acceptable as personal judgement dictates.

The Ructe template syntax uses include directives to pull in all of the necessary files for a given template.  This is where minimizing non-demo files will help.
 
