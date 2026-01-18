class Solution {
public:
    long long largestSquareArea(vector<vector<int>>& bottomLeft, vector<vector<int>>& topRight) {
        long long ans = 0;
        int n = bottomLeft.size();
        for (int i = 0; i < n; i++) {
            for (int j = i + 1; j < n; j++) {
                int bottom_left = max(bottomLeft[i][0], bottomLeft[j][0]);
                int top_right = min(topRight[i][0], topRight[j][0]);
                if (bottom_left >= top_right)
                    continue;

                int bottom_right = max(bottomLeft[i][1], bottomLeft[j][1]);
                int top_left = min(topRight[i][1], topRight[j][1]);
                if (bottom_right >= top_left) 
                    continue;

                int width = min(top_right - bottom_left, top_left - bottom_right);
                ans = max(ans, 1LL * width * width);
            }
        }
        return ans;
    }
};