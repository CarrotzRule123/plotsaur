import { COLOR, PlotWindow } from "../mod.ts";

const plot = new PlotWindow("Plotsaur Canvas", 600, 600);
plot.addPlot({
    caption: {
        caption: "Plotsaur Canvas",
        style: { family: "sans-serif", size: 30 }
    },
    xLabelAreaSize: 0,
    yLabelAreaSize: 0,
    mesh: undefined,
    seriesLabel: undefined
})
plot.cartesian2D({
    type: "ranged",
    x_axis: { start: -1, end: 1 },
    y_axis: { start: -1, end: 1 },
})

plot.drawRect({
    style: COLOR.RED,
    points: [{ x: 100, y: 100 }, { x: 200, y: 200 }],
    filled: true
})
plot.drawCircle({
    style: COLOR.BLUE,
    size: 50,
    points: { x: 300, y: 150 },
    filled: true
})
plot.drawPolygon({
    style: COLOR.YELLOW,
    points: [{ x: 400, y: 100 }, { x: 400, y: 200 }, { x: 500, y: 200 }],
    filled: true
})
plot.drawText({
    color: COLOR.BLACK,
    points: { x: 100, y: 250 },
    text: "Some text here",
    style: { family: "sans-serif", size: 30 }
})
plot.drawPath({
    style: COLOR.BLACK,
    points: [{ x: 100, y: 300 }, { x: 500, y: 300 }],
})
plot.show()
