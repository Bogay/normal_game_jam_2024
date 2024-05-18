#define PY_SSIZE_T_CLEAN

#include <Python.h>
#include "../bullet.h"
PyObject *pModule = NULL;
PyObject *py_create_bullet_ptr = NULL;

PyObject* import_py_func(char *func)
{
    PyObject *pFunc = PyObject_GetAttrString(pModule, func);
    if (pFunc == NULL)
    {
        PyErr_Print();
        exit(-1);
    }
    return pFunc;
}
void init()
{
    Py_Initialize();
    PyObject *sys_path = PySys_GetObject("path");
    PyList_Append(sys_path, PyUnicode_FromString("py/"));
    
    pModule = PyImport_ImportModule("ffi");
    if (pModule == NULL)
    {
        PyErr_Print();
        exit(-1);
    }
    py_create_bullet_ptr = import_py_func("py_create_bullet");
}
void py_create_bullet(Bullet *bullet) {
    if (pModule == NULL)
        init();
    
    PyObject *pArgs, *pKargs, *pRes;
    
    pArgs = Py_BuildValue("(y#)", bullet, sizeof(Bullet));
    PyErr_Print();
    pKargs = Py_BuildValue("{}");
    PyErr_Print();
        
    pRes = PyObject_Call(py_create_bullet_ptr, pArgs, pKargs);
    Py_DECREF(pArgs);
    Py_DECREF(pKargs);
    printf("sz:%d\n", PyBytes_Size(pRes));
    
    Bullet *ptr = PyBytes_AsString(pRes);
    printf("speed:%d\n", ptr->speed);
    printf("damage_by_frame:%d\n", ptr->damage_by_frame);
    
}