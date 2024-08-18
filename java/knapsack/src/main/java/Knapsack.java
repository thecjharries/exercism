import java.util.List;

class Knapsack {

    Knapsack() {}

    int maximumValue(int maximumWeight, List<Item> items) {
        int size = items.size();
        if (0 == maximumWeight || 0 == size) {
            return 0;
        }
        int[][] weights = new int[size + 1][maximumWeight + 1];
        for (int index = 0; index <= size; index++) {
            weights[0][index] = 0;
        }
        for (int walk = 1; walk <= size; walk++) {
            Item currentItem = items.get(walk - 1);
            for (int weight = 0; weight <= maximumWeight; weight++) {
                if (currentItem.weight > weight) {
                    weights[walk][weight] = weights[walk - 1][weight];
                } else {
                    weights[walk][weight] = Math.max(weights[walk - 1][weight], currentItem.value + weights[walk - 1][weight - currentItem.weight]);
                }
            }
        }
        return weights[size][maximumWeight];
    }

}
