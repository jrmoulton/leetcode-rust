#include "stdlib.h"

struct ListNode {
    int val;
    struct ListNode* next;
};

struct ListNode* mergeTwoLists(struct ListNode* list1, struct ListNode* list2) {
    struct ListNode temp;
    struct ListNode* movptr = &temp;
    while (list1 != 0 || list2 != 0) {
        if (list1 == 0) {
            movptr->next = list2;
            movptr = movptr->next;
            list2 = list2->next;
        } else if (list2 == 0) {
            movptr->next = list1;
            movptr = movptr->next;
            list1 = list1->next;
        } else {
            if (list1->val < list2->val) {
                movptr->next = list1;
                movptr = movptr->next;
                list1 = list1->next;
            } else {
                movptr->next = list2;
                movptr = movptr->next;
                list2 = list2->next;
            }
        }
    }
    return temp.next;
}

int main() {
    int arr1[3] = {1, 2, 4};
    int arr2[3] = {1, 3, 4};
    struct ListNode list1;
    struct ListNode list2;
    struct ListNode* l1p = &list1;
    struct ListNode* l2p = &list2;
    for (int i = 0; i < 3; i++) {
        l1p->val = arr1[i];
        l1p->next = malloc(sizeof(struct ListNode));
        l1p = l1p->next;
        l2p->val = arr2[i];
        l2p->next = malloc(sizeof(struct ListNode));
        l2p = l2p->next;
    }
    mergeTwoLists(&list1, &list2);
    return 0;
}
