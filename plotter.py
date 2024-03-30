from dash_extensions import WebSocket
from dash import Dash, html, dcc, Input, Output
import plotly.express as px
import os

if os.getenv("PROD", "").lower() == "true":
    url = "ws://localhost/ws"
else:
    url = "ws://localhost:3000/ws"


f = px.scatter(x=[1], y=[1])
f.update_traces(mode="markers+lines")
f.update_xaxes(visible=False)
f.update_yaxes(visible=False)

update_graph = """function(msg) {
    if(!msg){return {};}
    const data = JSON.parse(msg.data);
    data.x.push(data.x[0]);
    data.y.push(data.y[0]);
    return {data: [{x: data.x, y: data.y, type: "scatter", mode: "markers+lines"}]}};
"""

app = Dash()
app.layout = html.Div(
    [
        WebSocket(id="ws", url=url),
        dcc.Graph(id="figure", figure=f),
    ]
)
app.clientside_callback(
    update_graph, Output("figure", "figure"), Input("ws", "message")
)


if __name__ == "__main__":
    app.run(host="0.0.0.0", port="8050", debug=True)
