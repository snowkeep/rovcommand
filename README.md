# ROV Command
This will be a programming game written in Rust, with Conrod as the GUI.  You will be programming a Remote Operated Vehicle, either a submarine or surface vessel, most like in Dyon.  It is heavily influenced by Robocode and Scalatron.

Current status:
   * does not compile
   * almost has a Conrod GUI window
   
Plans:
   * surface vehicle
      * moves faster than submarine
      * has gun, depth charge, radar (surface), sonar (subsurface) and flak
      * causes much damage, and sustains little damage on head on ram
   
   * submarine
      * moves slower than surface vehicle on water
      * moves slower submerged than on surface
      * has gun (surface) and torpedos (submerged), radar (surface), sonar (subsurface)
      * nearby depth charge explosion is lethal
      * cannot be seen when submerged, not moving and not firing
      * can be found by sonar when moving or firing torpedos
      * when submerged, can find other submerged sub by ping - gives up location
      * causes much damage, and sustains medium damage on head on ram
      
   * torpedos
      * cause much more damage than guns
      * can be tracked by sonar and stopped by flak
      
   * guns
      * cannot be tracked or seen
      * cause much less damage than torpedoes
      
   * shoals, islands
      * block sonar
      * maybe block radar
      
   * energy budget (see Scalatron)
      * all vessels have a fixed energy budget
      * all movement, firing, surfacing and submerging, and damage reduces budget
      
Design details:
   * I hope to decouple the simulation engine from the GUI to enable
      * possible web GUI (see scalatron)
      * client/server model for multiple players
   * ROV coding:
      * main target is Dyon (I may change my mind)
      * native rust, compiled to .so should be possible

Other thoughts:
   * lua, javascript plugin
   * some kind of graphical programmming, like blockly
   
   
