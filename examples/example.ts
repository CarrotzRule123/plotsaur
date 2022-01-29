import { PlotChart, PlotWindow } from "../mod.ts";

const plot = new PlotWindow("Hello World", 400, 600);
const chart = new PlotChart({
    margin: 20,
    caption: {
        caption: "Plotsaur Chart",
        style: {
            family: "sans-serif",
            size: 30
        }
    },
    xLabelAreaSize: 40,
    yLabelAreaSize: 50,
    cartesian2D: {
        x_axis: { start: 0, end: 50 },
        y_axis: { start: 0, end: 1 },
    },
    mesh: {
        xLabels: 40,
        yLabels: 5,
        xDesc: "Seconds",
        yDesc: "% Busy",
        axisDescStyle: {
            family: "sans-serif",
            size: 15
        }
    },
    seriesLabel: {
        backgroundStyle: { r: 255, g: 255, b: 255, a: 1 },
        borderStyle: { r: 0, g: 0, b: 0, a: 1 }
    }
})
plot.addPlot(chart)
plot.show()
