import { PlotChart, PlotWindow } from "../mod.ts";

const plot = new PlotWindow("Hello World", 400, 600);
const chart = new PlotChart({
    margin: 50
})
plot.addPlot(chart)
plot.show()
