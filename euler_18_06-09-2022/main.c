#include <stdio.h>
#include <stdlib.h>

struct Node {
	int value;
	struct Node* left;
	struct Node* right;
};

struct Node* addNode(int value) {
	struct Node* node = (struct Node*)malloc(sizeof(struct Node));
	node->value = value;
	node->left = NULL;
	node->right = NULL;
	return (node);
}

/* Given a binary tree, print its nodes in preorder*/
void printPreorder(struct Node* node) {
    if (node == NULL)
        return;
    printf("%d ", node->value);
    printPreorder(node->left);
    printPreorder(node->right);
}

struct Node* generateTree() {
	struct Node* node = addNode(75);
	node->left = addNode(95);
	node->right = addNode(64);
	node->left->left = addNode(17);
	node->left->right = addNode(47);
	node->right->left = node->left->right;
	node->right->right = addNode(82);
	node->left->left->left = addNode(18);
	node->left->left->right = addNode(35);
	node->left->right->left = node->left->left->right;
	node->left->right->right = addNode(87);
	node->right->right->left = node->left->right->right;
	node->right->right->right = addNode(10);
	return (node);
}

int main() {
	struct Node* tree = generateTree();

	printPreorder(tree);

	return 0;
}
