#include "bullet.h"
extern Bullet *create_bullet(char **spell, int cnt);
int main()
{
    char *test[]={
        "python",
        /*
        "C",
        "c plus plus",
        "go",
        */
    };
    create_bullet(test, sizeof(test)/sizeof(char*));
}