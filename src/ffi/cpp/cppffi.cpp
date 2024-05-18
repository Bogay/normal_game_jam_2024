#include <iostream>
#include "../bullet.h"
void cpp_create_bullet(Bullet* bullet, int stacking)
{
    std::cout << "cpp_create_bullet" << std::endl;
    bullet->damage *= stacking;
    bullet->mp_cost += stacking;
}