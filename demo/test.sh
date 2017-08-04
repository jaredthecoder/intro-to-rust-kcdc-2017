echo "=========================================================="
echo "Running load tester for $1 and with Secure ID size of $2!"
echo "=========================================================="

wrk -t12 -c400 -d $1 http://localhost:8000/generate\?size\=$2
