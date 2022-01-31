import { COLOR, PlotWindow } from "../mod.ts";

const plot = new PlotWindow("Plotsaur Canvas", 600, 600);
plot.addPlot({
    caption: {
        caption: "Plotsaur Canvas",
        style: {
            family: "sans-serif",
            size: 30
        }
    },
    xLabelAreaSize: 0,
    yLabelAreaSize: 0,
    mesh: undefined,
    seriesLabel: undefined
})
plot.cartesian2D({
    x_axis: { start: -1, end: 1 },
    y_axis: { start: -1, end: 1 },
})

plot.drawRect({
    style: COLOR.RED,
    points: [{ x: 100, y: 100 }, { x: 200, y: 200 }],
    filled: true
})
plot.show()
