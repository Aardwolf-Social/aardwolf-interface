# aardwolf-interface
This is a dedicated repository for the User Interface (Front End) components of the Aardwolf Project :)

Ideally the interface being built here should be able to stand on its own.  This allows developers to play around, and test out ideas 
without having to build the full rust project.  Once the visual components have been vetted, they will of course need to be "backported" into the main repository
which is here: [Aardwolf-Social](https://github.com/aardwolf-social/aardwolf).
<br />

One of the goals of the project overall is to keep it simple, and secure.  That means trying to minimize the use of iFrames, JavaScript, and other scripting 
languages.  Thankfully HTML5, and the primary element library [Bulma.io](https://bulma.io) are able to provide dynamic layouts using only HTML, and CSS. 
Additionally, icons provided by [ForkAwesome](https://forkawesome.github.io/Fork-Awesome/) (a FOSS alternative to that other popular icon library),
is also free of JavaScript. 
<br />

**Directory Structure**<br />
There are two primary directories for the app `templates` which houses the bulk of the HTML files, and `web` which is where the styling, and static assets go.  [Click here for a visual depitiction](https://raw.githubusercontent.com/Aardwolf-Social/aardwolf-interface/banjo/update-readme.md/DIRECTORY_STRUCTURE.md)

<strong>UPDATE!</strong> Yew Framework
There is now -also- a directory called `aardwolf-yew-app`.  This is the `root` directory for the `Yew Framework` development.

**File Naming Convention**
The current naming convention is: `directoryname_description.html`
This is done to be able to find, and organize files quickly while troubleshooting.  The current exception to this rule is the "miscellaneous" parts which go into /templates/elements.  Even then those may have a better place elsewhere.

**Developer Resources**

***InVision***<br />
Wireframes have been posted to InVision, which can also be used for advanced prototyping:<br />
[InVision - Aardwolf Wireframes](https://invis.io/H3OTASXPMSY)

***CodePen***<br />
Banjo also has a CodePen account here: <br />
[CodePen](https://codepen.io/BanjoFox/)

***Contact***<br />
Here is a couple of ways to get in touch <br />
* Aardwolf Developer Chat: [#aardwolf-discussion:matrix.org](https://riot.im/app/#/room/#aardwolf-discussion:matrix.org)
* Message Banjo on Mastodon: [@banjofox2@hackers.town](@banjofox2@hackers.town)


