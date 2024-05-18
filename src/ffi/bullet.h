#include <stdbool.h>
#pragma pack(1)
typedef struct __RGBA__
{
    char r;
    char g;
    char b;
    char a;
} RGBA;
typedef struct __Bullet__
{
    int damage;
    int count;
    int speed;
    int hp;
    RGBA color;
    bool damage_by_frame;
    int mp_cost;
} Bullet;
#pragma unpack()