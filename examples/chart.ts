import { COLOR, PlotChart, PlotWindow } from "../mod.ts";

const plot = new PlotWindow("Plotsaur Chart", 600, 600);
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
    yLabelAreaSize: 40,
    cartesian2D: {
        x_axis: { start: -1, end: 1 },
        y_axis: { start: -1, end: 1 },
    },
    mesh: {
        xLabels: 10,
        yLabels: 10,
        xDesc: "X Axis",
        yDesc: "Y Axis",
        axisDescStyle: {
            family: "sans-serif",
            size: 15
        }
    },
    seriesLabel: {
        backgroundStyle: COLOR.WHITE,
        borderStyle: COLOR.BLACK
    }
})
plot.addPlot(chart)

const data = []
for (let i = -50; i < 50; i += 1) {
    data.push(i / 50, (i / 50) * (i / 50) * (i / 50))
}
chart.plot(COLOR.RED, "y = x ^ 3", data)
plot.show()
