import { COLOR, PlotWindow } from "../mod.ts";

const plot = new PlotWindow("Plotsaur Bar Chart", 600, 600);
plot.addPlot({
    caption: {
        caption: "Plotsaur Bar Chart",
        style: { family: "sans-serif", size: 30 }
    },
    mesh: {
        xDesc: "X Axis",
        yDesc: "Y Axis",
    },
    seriesLabel: undefined
})
plot.cartesian2D({
    type: "valuesX",
    x_axis: ["true", "false"],
    y_axis: { start: 0, end: 12 },
})

const data = [5, 10]
plot.plotHistogram({
    color: COLOR.RED,
    filled: true
}, data)
plot.show()
