echo "---"
echo "Rustack setup script"
echo "---"
echo

read -p 'Project name (snake_case): ' project_name
read -p 'Database name: ' db_name
read -p 'DB user: ' db_user
read -sp 'DB pass: ' db_pass
echo

rm .env

# Generate .env
echo "DATABASE_URL=postgres://$db_user:$db_pass@localhost/$db_name" >> .env
echo -e "APP_DEBUG=1\nRUN_MODE=development\nRUST_LOG=debug" >> .env
echo "Generated dot env file."
