FROM python:alpine

WORKDIR /app

RUN pip install uv

COPY requirements.txt requirements.txt
RUN VIRTUAL_ENV=$(python -c "import sys; print(sys.prefix)") uv pip install pandas
RUN VIRTUAL_ENV=$(python -c "import sys; print(sys.prefix)") uv pip install -r requirements.txt

COPY plotter.py plotter.py

CMD ["python", "plotter.py"]