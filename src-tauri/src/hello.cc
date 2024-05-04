#include "hello.h" // correct
#include <iostream>
using namespace std;

namespace farzi
{
	namespace tauri
	{ // write a function to print hello world
		void print_hello_world()
		{
			cout << "Hl";
		};
	};
};