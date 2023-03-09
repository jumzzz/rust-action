import argparse
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

def get_args():
    parser = argparse.ArgumentParser()
    parser.add_argument('src', help='Source file')
    parser.add_argument('dst', help='Destination image of the plot.')
    return parser.parse_args()

if __name__ == '__main__':
    args = get_args()
    df = pd.read_csv(args.src)

    title = 'Number of Threads Spawned VS Delay'
    sns.set_style('darkgrid')
    sns.set(rc={'figure.figsize':(11.7,8.27)}) 
    
    scatter_plot = sns.scatterplot(data=df, x='number_of_threads', y='delay')

    scatter_plot.set(xlabel='Number of Threads', ylabel='Delay (milliseconds)', title=title)
    
    fig = scatter_plot.get_figure()
    fig.savefig(args.dst)

    plt.show()