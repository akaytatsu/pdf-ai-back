FROM python:3.7-stretch
ENV PYTHONUNBUFFERED 1
ENV PYTHONDONTWRITEBYTECODE 1

RUN mkdir -p /app
WORKDIR /app

RUN pip install --upgrade pip
COPY requirements.txt ./
RUN pip install -r requirements.txt

COPY ./ ./

ENTRYPOINT ["sh", "./docker_assets/entrypoint.sh"]

CMD ["gunicorn", "conf.asgi:application", "-b", "0.0.0.0:8080", "--access-logfile", "-", "--error-logfile", "-", "--log-level", "warning", "-k", "uvicorn.workers.UvicornWorker"]