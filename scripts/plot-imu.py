from bokeh.plotting import figure
from bokeh.io import output_file, show
from bokeh.layouts import column, row
from bokeh.palettes import Dark2_5 as palette

import pandas

import itertools 

from argparse import ArgumentParser

def make_plot(title, y_axis, ts, ys):
    ys_avg = ys.rolling(window=10).mean()

    colors = itertools.cycle(palette)

    f = figure(title=title, y_axis_label=y_axis, plot_width=800)
    f.circle(ts, ys, color=next(colors))
    f.line(ts, ys_avg, color=next(colors))

    return f

def main(args):
    # output_file(args.output_html)

    df = pandas.read_csv(args.input_csv, parse_dates=['ts'])

    ts = df['ts']

    ax = df['ax']
    ay = df['ay']
    az = df['az']

    gx = df['gx']
    gy = df['gy']
    gz = df['gz']

    f_ax = make_plot('IMU Acceleration X', 'Acceleration', ts, ax)
    f_ay = make_plot('IMU Acceleration Y', 'Acceleration', ts, ay)
    f_az = make_plot('IMU Acceleration Z', 'Acceleration', ts, az)

    f_gx = make_plot('IMU Gyro X', 'Rotation', ts, gx)
    f_gy = make_plot('IMU Gyro Y', 'Rotation', ts, gy)
    f_gz = make_plot('IMU Gyro Z', 'Rotation', ts, gz)

    c1 = column(f_ax, f_ay, f_az)
    c2 = column(f_gx, f_gy, f_gz)

    show(row(c1, c2))



if __name__ == '__main__':
    parser = ArgumentParser()
    parser.add_argument('-i', '--input-csv', required=True, help='Input CSV file')
    parser.add_argument('-o', '--output-html', required=False, default='output.html', help='Output HTML file')

    args = parser.parse_args()

    main(args)
