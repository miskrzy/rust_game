## Game
1. Main menu:
 - home screen
 - controls screen (just view)
2. Game:
 - enemies spawn initially and over time, chase the player
 - player can move with wasd or arrows
 - projectiles shoot automatically towards closest enemy (no homing)
 - player health regenerates
 - player death spawns a game over 
 - **projectiles spawn animated explosion on impact**
3. Pause Menu:
 - can pause unpause with esc
 - can resume or exit to main menu
5. Arena
 - arena is now larger than the screen
 - camera follows the player but only within the area of the arena
6. GUI:
 - score
 - health bar
7. Game over screen
 - **game over screen shows final score**
## Others
1. Release:
 - a github actions pipeline for building and creating a release
 - pipeline uses CHANGELOG.md for release description
 - **all assets are loaded at the startup compared to every frame**
