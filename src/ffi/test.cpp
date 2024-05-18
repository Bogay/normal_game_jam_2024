#include "bullet.h"
extern Bullet *create_bullet(char **spell, int cnt);
int main()
{
    char *test[]={
        "C",
        "c plus plus",
        "go",
        "go",
        "go",
    };
    create_bullet(test, sizeof(test)/sizeof(char*));
}