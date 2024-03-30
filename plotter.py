from dash_extensions import WebSocket
from dash import Dash, html, dcc, Input, Output
import plotly.express as px


f = px.scatter(x=[1], y=[1])
f.update_traces(mode="markers+lines")
f.update_xaxes(visible=False)
f.update_yaxes(visible=False)
f.layout.title = "TSP"

update_graph = """function(msg) {
    if(!msg){return {};}  // no data, just return
    const data = JSON.parse(msg.data);  // read the data
    return {data: [{x: data.x, y: data.y, type: "scatter", mode: "markers+lines"}]}};  // plot the data
"""

app = Dash()
app.layout = html.Div(
    [
        WebSocket(id="ws", url="ws://localhost:3000/ws"),
        dcc.Graph(id="figure", figure=f),
    ]
)
app.clientside_callback(
    update_graph, Output("figure", "figure"), Input("ws", "message")
)


if __name__ == "__main__":
    app.run_server(debug=True, use_reloader=False)
