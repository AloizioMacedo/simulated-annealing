import json
import plotly.graph_objects as go
from websockets.sync.client import connect
from dash import Dash, html, dcc, callback, Input, Output


f = go.FigureWidget()
f.add_scatter(x=[0, 1], y=[0, 1])
f.update_traces(mode="markers+lines")
f.layout.title = "TSP"


app = Dash()
app.layout = html.Div(
    [dcc.Graph(id="figure", figure=f), dcc.Interval(id="interval", interval=100)]
)


@callback(
    Output(component_id="figure", component_property="figure"),
    Input(component_id="interval", component_property="n_intervals"),
)
def hello(value):
    try:
        message = websocket.recv()
        points = json.loads(message)
        x, y = zip(*points["points"])

        f.data[0].x = x
        f.data[0].y = y

        return f
    except Exception:
        return f


if __name__ == "__main__":
    with connect("ws://localhost:3000/ws") as websocket:
        app.run_server(debug=True, use_reloader=False)
