import matplotlib.pyplot as plt
import numpy as np
import sys

if len(sys.argv) < 2:
    print("Usage: python3 rho.2d.plot.py <file>")
    sys.exit(1)

filename = sys.argv[1]
#print(f"--- Отладка: Начинаю работу с файлом: {filename} ---")

try:
    # 1. Читаем данные как в вашем примере (пропуская 3 строки)
    # Предполагаем колонки: row, col, rho, mnt, bz
    # Если в файле 5 колонок, unpack=True разложит их по переменным
    row, col, rho, mnt, bz = np.loadtxt(filename, unpack=True, skiprows=3)

    # 2. Определяем размеры сетки
    # Вычисляем количество уникальных элементов по осям
    # Для reshape нам нужно знать n_rows и n_cols
    n_rows = int(np.max(row) + 1)
    n_cols = int(np.max(col) + 1)

    # 3. Превращаем плоский массив rho в 2D матрицу
    # Важно: reshape должен соответствовать размерности (rows, cols)
    Z = rho.reshape(n_rows, n_cols)

    # 4. Визуализация
    fig, ax = plt.subplots(figsize=(8, 6))
    fig.suptitle(f'Field rho from {filename}')

    # Цветовая карта (Heatmap)
    # origin='lower' чтобы (0,0) был в левом нижнем углу
    im = ax.imshow(Z, origin='lower', extent=[col.min(), col.max(), row.min(), row.max()], 
                   cmap='viridis', aspect='auto')
    
    # Линии уровней (Contour)
    contours = ax.contour(Z, levels=10, colors='white', alpha=0.5, 
                          extent=[col.min(), col.max(), row.min(), row.max()])
    ax.clabel(contours, inline=True, fontsize=8)

    plt.colorbar(im, ax=ax, label='rho')
    ax.set_xlabel('Column')
    ax.set_ylabel('Row')

    plt.show(block=True)

except Exception as e:
    print(f"Error: {e}")
    # Выведем отладочную информацию, если что-то не так с размерами
    if 'rho' in locals():
        print(f"Data size: {len(rho)}, Expected: {n_rows}x{n_cols}")

