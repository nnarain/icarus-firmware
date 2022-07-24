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
    output_file(args.output_html)

    df = pandas.read_csv(args.input_csv)

    ts = df['ts']

    pitch = df['pitch']
    roll = df['roll']
    yaw = df['yaw']

    f_pitch = make_plot('Pitch', 'Pitch', ts, pitch)
    f_roll = make_plot('Roll', 'Roll', ts, roll)
    f_yaw = make_plot('Yaw', 'Yaw', ts, yaw)

    c1 = column(f_pitch, f_roll, f_yaw)
    show(c1)


if __name__ == '__main__':
    parser = ArgumentParser()
    parser.add_argument('-i', '--input-csv', required=True, help='Input CSV file')
    parser.add_argument('-o', '--output-html', required=False, default='output.html', help='Output HTML file')

    args = parser.parse_args()

    main(args)
