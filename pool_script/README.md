# PoolScript
## Thanks  SwiteFaster!

Player z : 0
Camera z : 16

Use BE.

### Begin Game Data
* B0: pos_x
* B1: pos_y
* B2: pos_z
* B3: player_x
* B4: player_y
* B5: player_z
* B10: circle collide

### Begin Pointer Data
* B0: const value (4B)
* B1: game value (1B)
* B2: script data value (1B)
* B3: script stack value (1B)
* B4: calc stack value
* B9: no data

### Begin File Data
* 4B : Version  
* 1B : f32 Data Count  
* 2B : Function Name Bytes
#### Function Data
* B0: end
* B1: loop
* B2: ret (pointer)
* B3: push_to_stack_top (pointer)
* B4: allocate
* B5: break (pointer)
* B6: wait (pointer)
* B10: move_up (pointer)
* B11: summon_e (name, xyz, hp, collide, args..., ai, ai_args)
* B12: summon_b (name, xyz, scale, angle, collide_name, args..., bullet_ai, args...)

* B16: kill self

* B20: store_f32 (pointer)
* B21: add +
* B22: sub -
* B23: mul *
* B24: DIV /
* B25: MOD %
* B26: eq ==
* B27: neq !=
* B28: lt <
* B29: gt >
* B30: le <=
* B31: ge >=
* B38: sin (src, dst)
* B39: cos (src, dst)

* B40: load_texture (name, ron)