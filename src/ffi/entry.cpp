#include "bullet.h"
#include <cstdio>
#include <cstring>
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
extern "C" void c_create_bullet(Bullet* bullet);
extern cpp_create_bullet(Bullet* bullet, int stacking);
#include "go/go_ffi.h"

__declspec(dllexport) Bullet *create_bullet(char **spell, int cnt)
{

    if (cnt == 0)
        return nullptr;
    puts("create bullet");
    auto *base = init_bullet();
    for (auto i = 0; i != cnt; ++i)
    {
        if (strcmp(spell[i], "C") == 0)
            c_create_bullet(base);
        else if (strcmp(spell[i], "c plus plus") == 0)
            cpp_create_bullet(base, i + 1);
        else if (strcmp(spell[i], "go") == 0)
        {
            GoSlice gs;
            gs.data = &base;
            gs.len = sizeof(Bullet);
            gs.cap = sizeof(Bullet);
            printf("b4 hp: %d\n", base->hp);
            go_ffi(gs);
            memcpy(base, gs.data, sizeof(Bullet));
            printf("aft hp: %d\n", base->hp);
        }
        else
            break;
    }
    return base;
}