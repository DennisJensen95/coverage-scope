FROM dennisjensen95/coverage-scope:v0.3.2

COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
