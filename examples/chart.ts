import { COLOR, PlotWindow } from "../mod.ts";

const plot = new PlotWindow("Plotsaur Chart", 600, 600);
plot.addPlot({
    caption: {
        caption: "Plotsaur Chart",
        style: { family: "sans-serif", size: 30 }
    },
    mesh: {
        xDesc: "X Axis",
        yDesc: "Y Axis",
    },
})
plot.cartesian2D({
    type: "ranged",
    x_axis: { start: -1, end: 1 },
    y_axis: { start: -1, end: 1 },
})

const data = []
for (let i = -50; i < 50; i += 1) {
    data.push(i / 50, (i / 50) * (i / 50) * (i / 50))
}
plot.plotLineSeries({
    color: COLOR.RED,
    label: "y = x ^ 3"
}, data)
plot.show()
