#include "../bullet.h"
#include <stdio.h>
void c_create_bullet(Bullet* bullet)
{
    puts("c_create_bullet");
    bullet->damage *= 2;
    bullet->mp_cost *= 2;
}