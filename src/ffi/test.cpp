#include "bullet.h"
extern Bullet *create_bullet(char **spell, int cnt);
int main()
{
    char *test="asd";
    create_bullet(&test, 1);
}