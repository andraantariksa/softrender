#include <SDL.h>
#include <stdexcept>
#include <iostream>
#include <array>
#include <algorithm>
#include "Buffer.hpp"

struct Coord2D
{
	unsigned short x;
	unsigned short y;
};

class Engine
{
public:
	SDL_Window* window = nullptr;

	Engine()
	{
		if (SDL_Init(SDL_INIT_VIDEO) < 0)
		{
			throw std::runtime_error("Failed init");
		}

		window = SDL_CreateWindow("Soft Render", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, 640, 480, SDL_WINDOW_SHOWN);
		if (!window)
		{
			std::cerr << SDL_GetError() << '\n';
		}
	}

	void run()
	{
		Buffer<float> bufferZ(640, 480);
		Buffer<int> bufferColor(640, 480);

		SDL_Surface* surface = SDL_GetWindowSurface(window);
		SDL_PixelFormat* format = SDL_AllocFormat(SDL_PIXELFORMAT_RGBA32);
		SDL_Surface* output = SDL_ConvertSurface(surface, format, 0);

		bool running = true;
		SDL_Event event;
		while (running)
		{
			while (SDL_PollEvent(&event))
			{
				if (event.type == SDL_QUIT)
				{
					running = false;
				}
			}
			
			bufferColor.clear();
			triangle(bufferColor);
			memcpy(surface->pixels, bufferColor.buffer.data(), 640 * 480 * 4);

			SDL_UpdateWindowSurface(window);
		}

		SDL_DestroyWindow(window);
		SDL_Quit();
	}

	void triangle(Buffer<int> &pixels)
	{
		std::array<Coord2D, 3> arr = {
			Coord2D { 100, 100 },
			Coord2D { 300, 150 },
			Coord2D { 200, 100}
		};
		std::sort(arr.begin(), arr.end(), [](Coord2D& a, Coord2D& b) { return a.y < b.y; });

		for (int a = 100; a < 300; a++)
		{
			pixels.setPixel(a, a, 0xFFFF0000);
		}

		if (arr[0].y == arr[1].y)
		{
			int x_left;
			int y_left;
			int x_right;
			int y_right;
			if (arr[0].x < arr[1].x)
			{
				x_left = arr[0].x;
				y_left = arr[0].y;
				x_right = arr[1].x;
				y_right = arr[1].y;
			}
			else
			{
				x_left = arr[1].x;
				y_left = arr[1].y;
				x_right = arr[0].x;
				y_right = arr[0].y;
			}

			int dx_left = abs(arr[2].x - x_left);
			int sx_left = x_left < arr[2].x ? 1 : -1;
			int dy_left = -abs(arr[2].y - y_left);
			int sy_left = y_left < arr[2].y ? 1 : -1;
			int err_left = dx_left + dy_left;
			int e2_left;

			int dx_right = abs(arr[2].x - x_right);
			int sx_right = x_right < arr[2].x ? 1 : -1;
			int dy_right = -abs(arr[2].y - y_right);
			int sy_right = y_right < arr[2].y ? 1 : -1;
			int err_right = dx_right + dy_right;
			int e2_right;

			while (true)
			{
				pixels.setPixel(x_left, y_left, 0xFFFF0000);
				pixels.setPixel(x_right, y_right, 0xFFFF0000);

				if (x_left == arr[2].x && x_right == arr[2].x && y_left == arr[2].y) break;

				e2_left = 2 * err_left;
				if (e2_left >= dy_left) { err_left += dy_left; x_left += sx_left; }
				if (e2_left <= dx_left) { err_left += dx_left; y_left += sy_left; }
			}
		}
		else if (arr[1].y == arr[2].y)
		{

		}
		else
		{

		}
	}

	void plot_line(Buffer<int>& pixels, int x0, int y0, int x1, int y1, int color)
	{
		int dx = abs(x1 - x0), sx = x0 < x1 ? 1 : -1;
		int dy = -abs(y1 - y0), sy = y0 < y1 ? 1 : -1;
		int err = dx + dy, e2; /* error value e_xy */

		for (;;) {  /* loop */
			pixels.setPixel(x0, y0, color);
			if (x0 == x1 && y0 == y1) break;
			e2 = 2 * err;
			if (e2 >= dy) { err += dy; x0 += sx; } /* e_xy+e_x > 0 */
			if (e2 <= dx) { err += dx; y0 += sy; } /* e_xy+e_y < 0 */
		}
	}

};
