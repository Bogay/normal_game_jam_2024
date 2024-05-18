#include "bullet.h"
#include <cstdio>
Bullet *init_bullet()
{
    auto b = new Bullet();
    b->color.a = 0xFF;
    b->color.r = 0xFF;
    b->color.g = 0xFF;
    b->color.b = 0xFF;
    
    b->count = 1;
    b->damage = 1;
    b->damage_by_frame = false;
    b->hp = 0;
    b->mp_cost = 1;
    b->speed = 1;

    return b;
}
Bullet *create_bullet(char **spell, int cnt)
{

    if (cnt == 0)
        return nullptr;
    puts("create bullet");
    return init_bullet();
}