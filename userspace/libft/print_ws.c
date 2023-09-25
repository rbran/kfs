/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   print_ws.c                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mypark <mypark@student.42seoul.kr>         +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2021/12/28 21:41:35 by mypark            #+#    #+#             */
/*   Updated: 2021/12/29 03:22:49 by mypark           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "kfs/libft.h"

int	print_ws(int len, char ws)
{
	int	n;

	if (len <= 0)
		return (0);
	n = len;
	while (n--)
		write(1, &ws, 1);
	return (len);
}