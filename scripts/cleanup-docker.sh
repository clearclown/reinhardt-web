#!/bin/bash
# Cleanup TestContainers Docker resources
#
# Usage:
#   cleanup-docker.sh list              List all TestContainers resources
#   cleanup-docker.sh all               Remove all resources (with confirmation)
#   cleanup-docker.sh all --force       Remove all resources (no confirmation)
#   cleanup-docker.sh containers        Remove containers only
#   cleanup-docker.sh volumes           Remove volumes only
#   cleanup-docker.sh networks          Remove networks only

set -e

LABEL_FILTER="label=org.testcontainers.managed-by=testcontainers"

show_usage() {
	echo "Usage: cleanup-docker.sh <command> [options]"
	echo ""
	echo "Commands:"
	echo "  list              List all TestContainers resources"
	echo "  all               Remove all resources (with confirmation)"
	echo "  all --force       Remove all resources (no confirmation)"
	echo "  containers        Remove containers only"
	echo "  volumes           Remove volumes only"
	echo "  networks          Remove networks only"
}

list_resources() {
	echo "=== TestContainers Resources ==="
	echo ""

	echo "Containers:"
	CONTAINERS=$(docker ps -a --filter "$LABEL_FILTER" --format "table {{.ID}}\t{{.Image}}\t{{.Status}}\t{{.Names}}" 2>/dev/null || true)
	if [ -z "$CONTAINERS" ] || [ "$CONTAINERS" = "CONTAINER ID   IMAGE     STATUS    NAMES" ]; then
		echo "  (none)"
	else
		echo "$CONTAINERS" | sed 's/^/  /'
	fi
	echo ""

	echo "Volumes:"
	VOLUMES=$(docker volume ls --filter "$LABEL_FILTER" --format "table {{.Name}}\t{{.Driver}}" 2>/dev/null || true)
	if [ -z "$VOLUMES" ] || [ "$VOLUMES" = "VOLUME NAME   DRIVER" ]; then
		echo "  (none)"
	else
		echo "$VOLUMES" | sed 's/^/  /'
	fi
	echo ""

	echo "Networks:"
	NETWORKS=$(docker network ls --filter "$LABEL_FILTER" --format "table {{.ID}}\t{{.Name}}\t{{.Driver}}" 2>/dev/null || true)
	if [ -z "$NETWORKS" ] || [ "$NETWORKS" = "NETWORK ID   NAME      DRIVER" ]; then
		echo "  (none)"
	else
		echo "$NETWORKS" | sed 's/^/  /'
	fi
}

remove_containers() {
	CONTAINERS=$(docker ps -aq --filter "$LABEL_FILTER" 2>/dev/null || true)
	if [ -n "$CONTAINERS" ]; then
		echo "Stopping containers..."
		docker stop $CONTAINERS 2>/dev/null || true
		echo "Removing containers..."
		docker rm $CONTAINERS 2>/dev/null || true
		echo "✅ Containers removed."
	else
		echo "No containers found."
	fi
}

remove_volumes() {
	VOLUMES=$(docker volume ls -q --filter "$LABEL_FILTER" 2>/dev/null || true)
	if [ -n "$VOLUMES" ]; then
		echo "Removing volumes..."
		docker volume rm $VOLUMES 2>/dev/null || true
		echo "✅ Volumes removed."
	else
		echo "No volumes found."
	fi
}

remove_networks() {
	NETWORKS=$(docker network ls -q --filter "$LABEL_FILTER" 2>/dev/null || true)
	if [ -n "$NETWORKS" ]; then
		echo "Removing networks..."
		docker network rm $NETWORKS 2>/dev/null || true
		echo "✅ Networks removed."
	else
		echo "No networks found."
	fi
}

remove_all() {
	local force=$1

	CONTAINER_COUNT=$(docker ps -aq --filter "$LABEL_FILTER" 2>/dev/null | wc -l | tr -d ' ')
	VOLUME_COUNT=$(docker volume ls -q --filter "$LABEL_FILTER" 2>/dev/null | wc -l | tr -d ' ')
	NETWORK_COUNT=$(docker network ls -q --filter "$LABEL_FILTER" 2>/dev/null | wc -l | tr -d ' ')
	TOTAL=$((CONTAINER_COUNT + VOLUME_COUNT + NETWORK_COUNT))

	if [ "$TOTAL" -eq 0 ]; then
		echo "✅ No TestContainers resources found."
		exit 0
	fi

	echo "=== TestContainers Resources to Remove ==="
	echo "  Containers: $CONTAINER_COUNT"
	echo "  Volumes:    $VOLUME_COUNT"
	echo "  Networks:   $NETWORK_COUNT"
	echo "  Total:      $TOTAL"
	echo ""

	if [ "$force" != "true" ]; then
		read -p "Remove all resources? [y/N] " -n 1 -r
		echo ""
		if [[ ! $REPLY =~ ^[Yy]$ ]]; then
			echo "Cancelled."
			exit 0
		fi
	fi

	remove_containers
	remove_volumes
	remove_networks

	echo ""
	echo "✅ Cleanup complete."
}

# Main
case "${1:-}" in
	list)
		list_resources
		;;
	all)
		if [ "${2:-}" = "--force" ]; then
			remove_all true
		else
			remove_all false
		fi
		;;
	containers)
		remove_containers
		;;
	volumes)
		remove_volumes
		;;
	networks)
		remove_networks
		;;
	-h|--help|"")
		show_usage
		;;
	*)
		echo "Unknown command: $1"
		show_usage
		exit 1
		;;
esac
