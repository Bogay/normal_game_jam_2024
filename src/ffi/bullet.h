struct RGBA
{
    char r;
    char g;
    char b;
    char a;
};
struct Bullet
{
    int damage;
    int count;
    int speed;
    int hp;
    RGBA color;
    bool damage_by_frame;
    int mp_cost;
};