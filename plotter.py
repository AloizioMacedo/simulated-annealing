import json
import websocket
import rel
import time
import plotly.graph_objects as go
import dash

f = go.FigureWidget()
f.add_scatter(x=[0, 1], y=[0, 1])
f.update_traces(mode="markers+lines")
f.layout.title = "TSP"


def on_message(ws: websocket.WebSocket, message: str):
    points = json.loads(message)
    x, y = zip(*points["points"])
    print("JJJJJJJ")

    f.data[0].x = x
    f.data[0].y = y
    time.sleep(1)

    f.show()


def on_error(ws: websocket.WebSocket, error):
    print(error)


def on_close(ws: websocket.WebSocket, close_status_code, close_msg):
    print("### closed ###")


def on_open(ws: websocket.WebSocket):
    print("Opened connection")


if __name__ == "__main__":
    websocket.enableTrace(True)

    ws = websocket.WebSocketApp(
        "ws://localhost:3000/ws",
        on_open=on_open,
        on_message=on_message,
        on_error=on_error,
        on_close=on_close,
    )

    ws.run_forever(
        dispatcher=rel, reconnect=5
    )  # Set dispatcher to automatic reconnection, 5 second reconnect delay if connection closed unexpectedly
    rel.signal(2, rel.abort)  # Keyboard Interrupt
    rel.dispatch()
