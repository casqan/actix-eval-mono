import argparse
import json
import matplotlib.pyplot as plt
import numpy as np

def create_file_name(metric_name):
    return "./out/" + metric_name.replace(":", "-").replace("{", "_").replace("}", "") + ".png"

def plot_trend_metric(metric_name, values1, values2=None):
    """Plots the trend metrics (like avg, min, max, etc.) for a given metric."""
    stats = ['min', 'med', 'avg', 'max', 'p(90)', 'p(95)']
    data1 = [values1[stat] for stat in stats]

    plt.figure()
    y_pos = np.arange(len(stats))
    bars1 = plt.barh(y_pos + 0.2, data1, color='lightgreen', label='Spring', height=0.4)

    # Initialize for stacking
    if values2:
        data2 = [values2[stat] for stat in stats]
        bars2 = plt.barh(y_pos - 0.2, data2, color='indigo', label='Actix', height=0.4)

    plt.yticks(y_pos, stats)
    plt.title(f'{metric_name} Trend')
    plt.xlabel('Value')
    plt.ylabel('Statistic')
    plt.grid(True)
    plt.legend()

    # Display values on the bars for File 1
    for bar in bars1:
        xval = bar.get_width()
        plt.text(xval, bar.get_y() + bar.get_height()/2, round(xval, 2), va='center', ha='left')

    # Display values on the bars for File 2
    if values2:
        for bar in bars2:
            xval = bar.get_width() + bar.get_x()
            plt.text(xval, bar.get_y() + bar.get_height()/2, round(xval, 2), va='center', ha='left')

    plt.savefig(create_file_name(metric_name))

def plot_counter_metric(metric_name, values1, values2=None):
    """Plots the counter metrics like count and rate."""
    labels = list(values1.keys())
    data1 = list(values1.values())

    plt.figure()
    y_pos = np.arange(len(labels))
    bars1 = plt.barh(y_pos + 0.2, data1, color='lightgreen', label='Spring', height=0.4)

    if values2:
        data2 = list(values2.values())
        bars2 = plt.barh(y_pos - 0.2, data2, color='indigo', label='Actix', height=0.4)


    plt.yticks(y_pos, labels)
    plt.title(f'{metric_name} Counter')
    plt.xlabel('Value')
    plt.ylabel('Metric')
    plt.grid(True)
    plt.legend()

    # Display values on the bars for File 1
    for bar in bars1:
        xval = bar.get_width()
        plt.text(xval, bar.get_y() + bar.get_height()/2, round(xval, 2), va='center', ha='left')

    # Display values on the bars for File 2
    if values2:
        for bar in bars2:
            xval = bar.get_width() + bar.get_x()
            plt.text(xval, bar.get_y() + bar.get_height()/2, round(xval, 2), va='center', ha='left')

    plt.savefig(create_file_name(metric_name))

def plot_gauge_metric(metric_name, values1, values2=None):
    """Plots the gauge metrics."""
    labels = list(values1.keys())
    data1 = list(values1.values())

    plt.figure()
    y_pos = np.arange(len(labels))
    bars1 = plt.barh(y_pos - 0.2, data1, color='lightgreen', label='Spring', height=0.4)

    if values2:
        data2 = list(values2.values())
        bars2 = plt.barh(y_pos - 0.2, data2, color='indigo', label='Actix', height=0.4)


    plt.yticks(y_pos, labels)
    plt.title(f'{metric_name} Gauge')
    plt.xlabel('Value')
    plt.ylabel('Metric')
    plt.grid(True)
    plt.legend()

    # Display values on the bars for File 1
    for bar in bars1:
        xval = bar.get_width()
        plt.text(xval, bar.get_y() + bar.get_height()/2, round(xval, 2), va='center', ha='left')

    # Display values on the bars for File 2
    if values2:
        for bar in bars2:
            xval = bar.get_width() + bar.get_x()
            plt.text(xval, bar.get_y() + bar.get_height()/2, round(xval, 2), va='center', ha='left')

    plt.savefig(create_file_name(metric_name))

def main(file_path1, file_path2=None):
    # Load JSON data from the first file
    with open(file_path1, 'r') as file:
        data1 = json.load(file)

    data2 = None
    if file_path2:
        # Load JSON data from the second file if provided
        with open(file_path2, 'r') as file:
            data2 = json.load(file)

    metrics1 = data1["metrics"]
    metrics2 = data2["metrics"] if data2 else None

    for metric_name, metric_data1 in metrics1.items():
        metric_type = metric_data1["type"]
        values1 = metric_data1["values"]

        values2 = metrics2[metric_name]["values"] if metrics2 and metric_name in metrics2 else None

        if metric_type == "trend":
            plot_trend_metric(metric_name, values1, values2)
        elif metric_type == "counter":
            plot_counter_metric(metric_name, values1, values2)
        elif metric_type == "gauge":
            plot_gauge_metric(metric_name, values1, values2)


parser = argparse.ArgumentParser(
    prog='Actix-Eval GraphBuilder',
    description='A script to automatically generate graphs out k6 data')
parser.add_argument('springboot')
parser.add_argument('actix')

args = parser.parse_args()

main(args.springboot, args.actix)
