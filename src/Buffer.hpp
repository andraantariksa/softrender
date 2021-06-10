#include <array>
#include <vector>

template <typename T>
class Buffer
{
public:
	Buffer(size_t x_size, size_t y_size):
		x_size { x_size },
		y_size { y_size },
		buffer(x_size * y_size, 0)
	{
	}

	void setPixel(size_t x, size_t y, T pixel)
	{
		buffer[y * x_size + x] = pixel;
	}

	void clear()
	{
		memset(buffer.data(), 0, x_size * y_size * 4);
	}

	size_t x_size;
	size_t y_size;
	std::vector<T> buffer;
};
