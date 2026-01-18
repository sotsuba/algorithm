class Solution(object):

    def largestSquareArea(self, bottomLeft, topRight):
        """
        :type bottomLeft: List[List[int]]
        :type topRight: List[List[int]]
        :rtype: int
        """
        n = len(bottomLeft)
        ans = 0
        for i in range(n):
            for j in range(i + 1, n):
                bottom_left = max(bottomLeft[i][0], bottomLeft[j][0])
                top_right = min(topRight[i][0], topRight[j][0])
                
                if bottom_left >= top_right:
                    continue 
                
                bottom_right = max(bottomLeft[i][1], bottomLeft[j][1])
                top_left = min(topRight[i][1], topRight[j][1])

                if bottom_right >= top_left:
                    continue 
                
                width = top_right - bottom_left
                height = top_left - bottom_right
                ans = max(ans, min(width, height)**2) 
        return ans